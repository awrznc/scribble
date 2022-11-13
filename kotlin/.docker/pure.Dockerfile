FROM debian:11

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y curl

ENV KOTLIN_ARTIFACT_URL "https://github.com/JetBrains/kotlin/releases/download/v1.7.10/kotlin-native-linux-x86_64-1.7.10.tar.gz"
RUN curl -o /tmp/kotlin.tar.gz -OL ${KOTLIN_ARTIFACT_URL}
RUN tar zxvf /tmp/kotlin.tar.gz -C /tmp/ \
    && ln -s /tmp/kotlin-native-linux-x86_64-1.7.10 /usr/local/kotlin
ENV PATH "${PATH}:/usr/local/kotlin/bin/"

# Kotlinコンパイラはネイティブ向けにコンパイルしてくれるがjavaがないと動かない
RUN apt-get install -y openjdk-17-jdk
RUN kotlinc-native -version
