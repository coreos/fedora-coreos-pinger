//! Collect container runtime information
//!
//! Four container runtimes are monitored: docker, podman, crio, and systemd-nspawn
//! and the number of containers running by each runtime is extracted as follows, correspondingly:
//! `podman container ls | wc -l`
//! `docker contaienr ls | wc -l`
//! `crictl ps --output table | wc -l`
//! `machinectl list | grep "container" | grep "systemd-nspawn" | wc -l`
//!
//! And the system-wide information of each runtime is collected by running:
//! `podman info --format json`
//! `docker info --format '{{json .}}'`
//! `crictl info`

use failure::{self, bail, Fallible};
use serde::Deserialize;
use std::cmp;
use std::io::Write;
use std::process;

/// stores number of containers run by each container runtime
#[derive(Debug, Deserialize)]
pub(crate) struct ContainerCounts {
    /// number of containers run by podman
    /// extracted from `podman container ls | wc -l`
    podman: i32,
    /// number of containers run by docker
    /// extracted from `docker container ls | wc -l`
    docker: i32,
    /// number of containers run by crio
    /// extracted from `crictl ps --output table | wc -l`
    crio: i32,
    /// number of containers run by systemd-nspawn
    /// where CLASS=container SERVICE=systemd-nspawn
    /// extracted from `machinectl list | grep "container" | grep "systemd-nspawn" | wc -l`
    systemd_nspawn: i32,
}

/// stores system-wide information from container runtimes
#[derive(Debug, Deserialize)]
pub(crate) struct ContainerInfo {
    /// output of `podman info --format json`
    podman: String,
    /// output of `docker info --format '{{json .}}'` if dockerd is running
    docker: String,
    /// output of `crictl info`, default format is json
    crio: String,
}

/// wrapper struct for container counts and system-wide information
#[derive(Debug, Deserialize)]
pub(crate) struct ContainerRT {
    counts: ContainerCounts,
    info: ContainerInfo,
}

/// function to run `${command} ${args}`
fn run_command(command: &str, args: &Vec<&str>) -> Result<process::Output, std::io::Error> {
    process::Command::new(command).args(args).output()
}

/// spawn a child process and pass the output of previous command through pipe
fn spawn_child(
    input: &str,
    command: &str,
    args: Vec<&str>,
) -> Result<process::Output, std::io::Error> {
    let mut child = process::Command::new(command)
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output()?;
    Ok(output)
}

/// return Error if the command failed
fn check_status(cmd: &str, output: &process::Output) -> Fallible<()> {
    if !(*output).status.success() {
        bail!(
            "{} failed:\n{}",
            cmd,
            String::from_utf8_lossy(&(*output).stderr)
        );
    }
    Ok(())
}

impl ContainerRT {
    pub(crate) fn new() -> ContainerRT {
        ContainerRT {
            counts: ContainerCounts {
                podman: Self::rt_count_running("podman").unwrap_or(0),
                docker: Self::rt_count_running("docker").unwrap_or(0),
                crio: Self::rt_count_running("crictl").unwrap_or(0),
                systemd_nspawn: Self::rt_count_running("machinectl").unwrap_or(0),
            },
            info: ContainerInfo {
                podman: Self::rt_fetch_info("podman").unwrap_or("".to_string()),
                docker: Self::rt_fetch_info("docker").unwrap_or("".to_string()),
                crio: Self::rt_fetch_info("crictl").unwrap_or("".to_string()),
            },
        }
    }

    /// counts the number of running containers/lines in the output using `wc -l`
    fn rt_count_running(container_rt: &str) -> Fallible<i32> {
        if !ContainerRT::rt_is_running(container_rt).unwrap_or(false) {
            bail!("{} not running or not supported", container_rt);
        }

        let options = match container_rt {
            "podman" => vec!["container", "ls"],
            "docker" => vec!["container", "ls"],
            "crictl" => vec!["ps", "--output", "table"],
            "machinectl" => vec!["list"],
            _ => bail!("container runtime {} is not supported", container_rt),
        };

        // run `${command} ${args}`
        let mut output = run_command(container_rt, &options)?;
        check_status(
            format!("{} {}", container_rt, options.join(" ")).as_str(),
            &output,
        )?;
        let mut std_out = String::from_utf8(output.stdout)?;

        // for `machinectl` we need to count the number of running `container` that run by `systemd-nspawn` service
        // i.e. run `grep container | grep systemd-nspawn` before `wc -l`
        if let "machinectl" = container_rt {
            output = spawn_child(std_out.as_str(), "grep", vec!["container"])?;
            check_status(
                format!("{} {}", container_rt, options.join(" ")).as_str(),
                &output,
            )?;
            std_out = String::from_utf8(output.stdout)?;

            output = spawn_child(std_out.as_str(), "grep", vec!["systemd-nspawn"])?;
            check_status(
                format!("{} {}", container_rt, options.join(" ")).as_str(),
                &output,
            )?;
            std_out = String::from_utf8(output.stdout)?;
        };

        // count the number of (filtered) running containers
        output = spawn_child(std_out.as_str(), "wc", vec!["-l"])?;
        check_status(
            format!("{} {}", container_rt, options.join(" ")).as_str(),
            &output,
        )?;
        std_out = String::from_utf8(output.stdout)?
            .trim()
            .trim_end_matches("\n")
            .to_string();;

        let count: i32 = std_out.parse()?;
        Ok(cmp::max(0, count - 1))
    }

    /// fetch the system-wide information from containter runtime (docker, podman, and crio)
    fn rt_fetch_info(container_rt: &str) -> Fallible<String> {
        if !ContainerRT::rt_is_running(container_rt).unwrap_or(false) {
            bail!("{} not running or not supported", container_rt);
        }

        let options = match container_rt {
            "podman" => vec!["info", "--format", "json"],
            "docker" => vec!["info", "--format", "{{json .}}"],
            "crictl" => vec!["info"],
            _ => bail!("container runtime {} is not supported", container_rt),
        };

        // run `${command} ${args}`
        let output = run_command(container_rt, &options)?;
        check_status(
            format!("{} {}", container_rt, options.join(" ")).as_str(),
            &output,
        )?;
        let std_out = String::from_utf8(output.stdout)?;
        Ok(std_out)
    }

    fn rt_is_running(container_rt: &str) -> Fallible<bool> {
        match container_rt {
            "podman" => {
                let command = "podman";
                let options = vec!["info", "--format", "json"];
                let result = run_command(command, &options)?;
                check_status(
                    format!("{} {}", command, options.join(" ")).as_str(),
                    &result,
                )?;
                Ok(true)
            }
            "docker" => {
                let command = "docker";
                let options = vec!["info", "--format", "'{{json .}}'"];
                let result = run_command(command, &options)?;
                check_status(
                    format!("{} {}", command, options.join(" ")).as_str(),
                    &result,
                )?;
                Ok(true)
            }
            "crictl" => {
                let command = "crictl";
                let options = vec!["info"];
                let result = run_command(command, &options)?;
                check_status(
                    format!("{} {}", command, options.join(" ")).as_str(),
                    &result,
                )?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
