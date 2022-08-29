

#!/bin/bash

export POSTGRES_USER=israel
export POSTGRES_PASSWORD=laguan
export POSTGRES_DB_URL=0.0.0.0
export POSTGRES_DB=postgres
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_DB_URL}/${POSTGRES_DB}"

# We need to bind to 0.0.0.0 inside the container
# ROCKET_ENV production already forces this in the image normally
export ROCKET_ADDRESS=0.0.0.0
