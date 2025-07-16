FROM python:3.11-slim

RUN apt-get update && apt-get install -y curl 

WORKDIR /app

COPY app /app/app
COPY requirements.txt /app/

RUN pip install --no-cache-dir -r requirements.txt

EXPOSE 8080

CMD ["python", "-m", "app.main"]
