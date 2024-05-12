FROM node

WORKDIR /docker-env
COPY . .

CMD ["node", "--env-file=.env", "."]
