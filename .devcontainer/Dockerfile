FROM espressif/idf-rust:esp32_latest

USER root
RUN usermod -a -G dialout esp
# ARG USERNAME=vscode
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

