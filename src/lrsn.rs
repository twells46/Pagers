use std::fs::OpenOptions;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use termios::*;

pub fn send_page(num: u32) -> io::Result<()> {
    // Open the serial port
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_SYNC)
        .open("/dev/ttyACM0")?;

    let fd = f.as_raw_fd();

    // Get current terminal attributes and modify them
    let mut tty = Termios::from_fd(fd)?;

    // 9600 baud
    // NOTE: This is only applied after the `tcsetattr` call
    cfsetspeed(&mut tty, B9600)?;

    // Send 8 bit characters, no parity, no flow control, one stop bit
    tty.c_cflag &= !CSIZE;
    tty.c_cflag |= CS8;
    tty.c_cflag &= !(PARENB | PARODD);
    tty.c_iflag &= !(IXON | IXON | IXANY);
    tty.c_cflag &= !CSTOPB;

    // Black magic to make it work
    tty.c_lflag = 0;
    tty.c_oflag = 0;
    tty.c_cflag |= CLOCAL | CREAD;
    tty.c_cc[VMIN] = 0;
    tty.c_cc[VTIME] = 10;

    tcsetattr(fd, TCSANOW, &tty)?;

    // let cmd = format!("CPG,{},1,4\n", num);
    let cmd = format!("CPG,{},1,4\n", num);

    let written = f.write(cmd.as_bytes())?;
    if written == 0 {
        return Err(Error::new(ErrorKind::WriteZero, "Wrote zero bytes"));
    }

    let mut buf: [u8; 32] = [0; 32];
    f.read(&mut buf)?;
    match std::str::from_utf8(&buf) {
        Ok(s) => String::from(s),
        Err(why) => format!("Not valid UTF-8: {}", why),
    };

    Ok(())

    // I would love to have the extra error reporting,
    // but something about the serial connection is not reliable enough
    // and sometimes wrongly reports failure.
    /*
    if resp.contains("CPG") {
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::NotFound,
        format!("Response: {}", resp),
    ))
    */
}
