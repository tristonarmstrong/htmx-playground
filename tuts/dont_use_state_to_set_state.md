# Don't use state to set state

This is very nasty stuff
```tsx
const {someHookState} = useSomeHook()
const [someState, setSomeState] = usState()

useEffect(() => {
	if (!someHookState) return
	setSomeState(someHookState)
}, [someHookState])

return (
	<div>{someState}</div>
)
```
It essentially causes two completely different rerenders unnecessarily. You may be thinking, well thats just stupid, ofcourse i wouldnt do that!  


You'd be VERY surprised. I see this all the time, all over the place. Makes absolutely no sense to me. 

All you need to do.. literally.. is this:
```tsx
const {someHookState} = useSomeHook()

return (
	<div>{someHookState}</div>
)
```

This is MUCH cleaner. Now you may be thinking, "Well what if someHookState is falsy". Then check for that like you normally would, dumbass.