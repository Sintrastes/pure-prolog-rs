FROM rustembedded/cross
MAINTAINER "Nathan Bedell" "nbedell@tulane.edu"
Add . /opt/runtime/
WORKDIR /opt/runtime/
RUN cross --release --target armv7-unknown-linux-gnueabihf
RUN cross --release --target x86_64-pc-windows-gnu