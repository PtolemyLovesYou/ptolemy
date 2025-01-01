#!/bin/bash

if [[ "$PTOLEMY_ENV" == "STAGE" || "$PTOLEMY_ENV" == "PROD" ]]; then
    echo "Running migrations in $PTOLEMY_ENV environment..."
    
    max_attempts=5
    attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        echo "Migration attempt $attempt of $max_attempts..."
        diesel migration run --database-url "postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB"
        
        if [ $? -eq 0 ]; then
            echo "Migration successful!"
            break
        else
            echo "Migration attempt $attempt failed!"
            
            if [ $attempt -eq $max_attempts ]; then
                echo "All migration attempts failed!"
                exit 1
            fi
            
            echo "Retrying in 1 second..."
            sleep 1
            ((attempt++))
        fi
    done
fi

exec /usr/local/bin/api
