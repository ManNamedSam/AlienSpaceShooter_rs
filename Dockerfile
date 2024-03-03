FROM python:3.10-slim-buster

WORKDIR /app

# Copy the entire pkg directory to /app/pkg in the container.
COPY ./static ./static

# Copy the local index.html file to the /app directory in the container.
COPY ./index.html .

# Copy the entire assets directory to /app/assets in the container.
COPY ./assets ./assets

EXPOSE 8080 

CMD ["python3", "-m", "http.server","8080"]
