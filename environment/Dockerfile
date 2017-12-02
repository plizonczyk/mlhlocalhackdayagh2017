#FROM python:3.6-jessie
FROM ubuntu:17.10

RUN apt update && apt upgrade && apt install -y python3
# COPY listeners.py listeners.py
COPY multiple_ports.sh multiple_ports.sh

# TCP 50123
# UDP 50321

# CMD ["./multiple_ports.sh"]
CMD ["/bin/bash"]
