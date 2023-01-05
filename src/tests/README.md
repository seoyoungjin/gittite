Test can be done in one of the following repositories.

1. Repository create or already created according to the test situation
2. mockIPC with typescript or json
3. fake invoke

# Unit test

Quasar makes component warings related with  VTU_COMPONENT.
So, I'll move logic to pinia store instead of vue component.
Then, we can test with mockIPC.

# Tauri driver

End-to-End testing

# In Developement

We can make test data with fakeInvoke(). 
Especially thih will be helpful to following cases.

- huge log data
- progress data

