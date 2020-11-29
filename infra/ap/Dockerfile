FROM adoptopenjdk:11-jre-hotspot
LABEL maintainer="yuimarl"

ENV LANG ja_JP.UTF8
ENV LC_ALL ja_JP.UTF8
RUN ln -sf /usr/share/zoneinfo/Japan /etc/localtime

VOLUME /tmp
ARG JAR_FILE
COPY ${JAR_FILE} yuimarl.jar

ENTRYPOINT ["java", \
  "-verbose:gc", \
  "-Xlog:gc*:stdout:time,uptime,level,tags", \
  "-Djava.security.egd=file:/dev/./urandom", \
  "-jar", \
  "/yuimarl.jar"]
