use std::io;

use tokio::process::Child;
use tokio::process::Command;

pub struct ChildGuard {
    #[cfg(windows)]
    job: windows_sys::Win32::Foundation::HANDLE
}

impl ChildGuard {
    pub fn prepare(command: &mut Command) {
        command.kill_on_drop(true);

        #[cfg(target_os = "linux")]
        {
            let parent = std::process::id() as libc::pid_t;

            // Runs between fork and exec, so only async-signal-safe calls are allowed.
            unsafe {
                command.pre_exec(move || {
                    if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) == -1 {
                        return Err(io::Error::last_os_error());
                    }

                    if libc::getppid() != parent {
                        return Err(io::Error::from_raw_os_error(libc::ESRCH));
                    }

                    Ok(())
                });
            }
        }
    }

    #[cfg(windows)]
    pub fn attach(child: &Child) -> io::Result<Self> {
        use std::mem;
        use std::ptr;

        use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
        use windows_sys::Win32::System::JobObjects::CreateJobObjectW;
        use windows_sys::Win32::System::JobObjects::JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        use windows_sys::Win32::System::JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION;
        use windows_sys::Win32::System::JobObjects::JobObjectExtendedLimitInformation;
        use windows_sys::Win32::System::JobObjects::SetInformationJobObject;

        let process = child
            .raw_handle()
            .ok_or_else(|| io::Error::other("child has already exited"))?;

        unsafe {
            let job = CreateJobObjectW(ptr::null(), ptr::null());

            if job.is_null() {
                return Err(io::Error::last_os_error());
            }

            let guard = Self { job };
            let mut information: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = mem::zeroed();
            information.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

            let configured = SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                (&raw const information).cast(),
                mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32
            );

            if configured == 0 || AssignProcessToJobObject(job, process.cast()) == 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(guard)
        }
    }

    // macOS has no kernel mechanism tying a child to its parent, so the bot only dies with
    // the wrapper through kill_on_drop, on a clean exit.
    #[cfg(not(windows))]
    pub fn attach(_child: &Child) -> io::Result<Self> {
        Ok(Self {})
    }
}

#[cfg(windows)]
impl Drop for ChildGuard {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.job);
        }
    }
}
