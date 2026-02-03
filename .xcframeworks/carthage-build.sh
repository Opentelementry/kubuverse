if [ -f "Cartfile.resolved" ]; then
  # Only runs bootstrap if dependencies are missing or out of date
  /usr/local/bin/carthage bootstrap --platform iOS --use-xcframeworks --cache-builds
else
  echo "No Cartfile.resolved found. Skipping Carthage build."
fi
