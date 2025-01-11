#!/bin/bash

if [[ "${PTOLEMY_ENABLE_MIGRATIONS:-true}" == "true" ]]; then
    DB_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB"
    echo "Starting migration process..."
    
    max_attempts=5
    attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        # TODO: Check if database is healthy. The sleep here is temporary
        sleep 1
        echo "Attempt $attempt of $max_attempts..."
        
        pending=$(diesel migration pending --database-url "$DB_URL")
        if [ $? -ne 0 ]; then
            echo "Failed to check pending migrations!"
            
            if [ $attempt -eq $max_attempts ]; then
                echo "All attempts to check migrations failed!"
                exit 1
            fi
            
            echo "Retrying in 1 second..."
            sleep 1
            ((attempt++))
            continue
        fi
        
        if [ "$pending" == "true" ]; then
            echo "Running pending migrations..."
            diesel migration run --database-url "$DB_URL"
            
            if [ $? -eq 0 ]; then
                echo "Migration successful!"
                break
            else
                echo "Migration failed!"
                
                if [ $attempt -eq $max_attempts ]; then
                    echo "All migration attempts failed!"
                    exit 1
                fi
                
                echo "Retrying in 1 second..."
                sleep 1
                ((attempt++))
            fi
        else
            echo "No pending migrations to run"
            break
        fi
    done
fi

exec /usr/local/bin/api
