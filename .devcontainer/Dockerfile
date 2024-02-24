FROM espressif/idf-rust:esp32_latest

USER root
RUN usermod -a -G dialout esp
USER esp
RUN cargo install espflash@3.0.0-rc.1 cargo-espflash@3.0.0-rc.1 --force
RUN cargo install cargo-generate

# RUN cargo install espflash@3.0.0-rc.1
# RUN cargo install cargo-espflash@3.0.0-rc.1
# # ARG USERNAME=vscode
# ARG USER_UID=1001
# ARG USER_GID=$USER_UID

# # Create the user
# RUN groupadd -f --gid $USER_GID $USERNAME \
#     && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
#     #
#     # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
#     && apt-get update \
#     && apt-get install -y sudo \
#     && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
#     && chmod 0440 /etc/sudoers.d/$USERNAME
# RUN usermod -a -G dialout vscode
# # ********************************************************
# # * Anything else you want to do like clean up goes here *
# # ********************************************************

# # [Optional] Set the default user. Omit if you want to keep the default as root.
# USER $USERNAME

