# Stoic Euclido  

This is a secure wallet for storing digital assets(spl tokens) offering extra layers of security 
such as delayed or restricted access and withdrawal whitelist.   
This is an automated smart contract for storing digital assets, delaying their withdrawals for a particular period of time, 
the default is 72 hours.   

### Features  
- Time delays before withdrawals are processed(default if not set is 72 hrs). You can lock your asset to control access.  
- Withdrawal whitelist. Transfers are only permitted to the wallet address that locked assets into it.  
- Multiple wallets can be used to lock assets in the vault.  
