FROM google/cloud-sdk:alpine

RUN apk --update --no-cache add openjdk11-jre

RUN gcloud config set auth/disable_ssl_validation True && \
gcloud components update

CMD ["gcloud", "emulators", "firestore", "start", "--host-port", "0.0.0.0:8080", "--quiet"]
