# unsh
Unsh, the un-shell. It just executes commands

## FAQ

### Q: What is unsh? Is it a shell?
Yeah, kinda. You can run commands with it like this:

```
¥ sudo systemctl status crond
-  (press RETURN)● crond.service - Command Scheduler
     Loaded: loaded (/usr/lib/systemd/system/crond.service; enabled; vendor preset: enabled)
     Active: active (running) since Mon 2021-05-10 17:58:41 PDT; 1 weeks 3 days ago
   Main PID: 1904 (crond)
      Tasks: 1 (limit: 18855)
     Memory: 1.9M
     CGroup: /system.slice/crond.service
             └─1904 /usr/sbin/crond -n
```

### Q: Awesome, so what other things can it do?
A: That's pretty much it. It passes argv to your program using standard shell argument parsing

But, you can't pipe the output of one program into another:

```
¥ echo "Hi there" | sed s/there/hi
Hi there | sed s/there/hi
exit code: 0
```

### Q: What? But that's a basic thing shells do.
A: Not this shell. This shell doesn't do that.

It doesn't do variables either:

```
¥ echo $PATH
$PATH
exit code: 0
```

### Q: What if I need to pipe the output of one command into another?
A: Use a real programming language. Shell scripting languages are just crippled programming languages.

You've got lots of options. Like python, or Rust.

Shells typically conflate programming environment and process execution. Unsh doesn't, it just does
command execution and leaves the programming to programming languages.

### Q: Why does the prompt have a ¥ instead of a $? Oh I get it.
A: Yeah, you got it.
