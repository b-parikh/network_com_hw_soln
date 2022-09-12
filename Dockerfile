FROM        ubuntu:focal

RUN         apt-get update \
                && apt-get install -y \
                    software-properties-common \
                    wget \
                && add-apt-repository -y ppa:ubuntu-toolchain-r/test \
                && apt-get update \
                && apt-get install -y \
                    make \
                    curl \
                    vim \
                    build-essential \ 

                # Build CMake from scratch for NNG dependency
#                && mkdir /tmp/cmake-install \
#                && wget https://github.com/Kitware/CMake/releases/download/v3.24.1/cmake-3.24.1.tar.gz -P /tmp/cmake-install \
#                && cd /tmp/cmake-install \
#                && tar -xf cmake-3.24.1.tar.gz \
#                && cd cmake-3.24.1 \
#                && ./bootstrap --no-qt-gui -- -DCMAKE_USE_OPENSSL=OFF \
#                && make && make install \
#                && cp ./bin/cmake /usr/local/bin \

                # install rust toolchain
                && curl https://sh.rustup.rs -sSf | bash -s -- -y \
                && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc 
                
                # CMake Version 3.24 (>3.16) should be installed for the NNG dependency
                # Copy appropriate files/binaries in from local directory
COPY            ./cmake_bin/cmake /usr/local/bin/cmake
COPY            ./cmake_bin/cpack /usr/local/bin/cpack
COPY            ./cmake_bin/ctest /usr/local/bin/ctest
COPY            ./cmake_bin/cmake-3.24 /usr/local/share/cmake-3.24
