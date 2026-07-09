// Run: wrangler kv:key put --binding=PACKAGES "apikey:<key>" '{"org":"atlas","role":"admin"}'
// Generate a secure key: node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"
// Then deploy: wrangler kv:key put --binding=PACKAGES "apikey:<your-key>" '{"org":"atlas","role":"admin"}'
console.log("To create an admin API key:");
console.log("1. Generate a key:  node -e \"console.log(require('crypto').randomBytes(32).toString('hex'))\"");
console.log("2. Store it:        wrangler kv:key put --binding=PACKAGES 'apikey:<key>' '{\"org\":\"atlas\",\"role\":\"admin\"}'");
console.log("3. Use it:          atlas publish . --api-key <key>");
