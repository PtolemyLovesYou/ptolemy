#!/bin/bash

if [[ "${PTOLEMY_ENABLE_MIGRATIONS:-true}" == "true" ]]; then
    DB_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB"
    echo "Checking for pending migrations..."
    
    pending=$(diesel migration pending --database-url "$DB_URL")
    if [ "$pending" == "true" ]; then
        max_attempts=5
        attempt=1
        
        while [ $attempt -le $max_attempts ]; do
            sleep 1
            echo "Migration attempt $attempt of $max_attempts..."
            diesel migration run --database-url "$DB_URL"
            
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
    else
        echo "No pending migrations to run"
    fi
fi

exec /usr/local/bin/api
