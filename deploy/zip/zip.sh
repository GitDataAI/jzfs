#!/bin/bash

echo "Creating zip archive..."

if [ -f "output.zip" ]; then
    echo "Deleting existing output.zip..."
    rm "output.zip"
fi

zip -r "output.zip" . -x "output.zip" "target/*" ".idea/*" "web/node_modules/*" "web/dist/*"

if [ $? -eq 0 ]; then
    echo "Successfully created output.zip"
else
    echo "Failed to create output.zip"
    exit 1
fi
