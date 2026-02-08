# thoughts on erlang

I am, by no means, an expert on erlang. To be honest I don't fully grasp the basic concepts yet. But I am fascinated by the ideas that 
I came across while exploring the docs and playing around with erlang and the OTP . So here I wanted to share a bit of my excitement and 
maybe lure you into the rabbit hole. But keep in mind that as I said I am no expert and everything I am saying should be taken with a chunk of salt.
Also it will be written the way I understand it. So there may be references to existing programming (or rust) concepts that aid me in grasping
the new information.

The main think that lead to me obsessing over erlang for a while is how different it is. It takes an approach to both parallelization and error 
handling.

## basics

Next to the usual primitive data types erlang has basic building blocks called "atoms". They are a name that can be compared to other atoms and that's
about it. So they are somewhat of a enum field but the enum is 
[non_exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#r-attributes.type-system.non_exhaustive.intro). The primary way of 
interacting with them is pattern matching. On the top of pattern matching: erlang supports pattern matching at a function level

```erlang 
prefix(m) -> "Mr.";
prefix(f) -> "Mrs.";
prefix(_) -> "Mx.".
```

## processes

Processes are a core concept of the concurrency model in erlang. They are not OS processes but rather processes in the runtime VM. But their 
behavior is very similar to the "normal" processes. They get initialized with a starting state that they hold and then wait for external calls

(in erlang they are received via message passing) that then get handled until the processes exits. Here we also see how useful atoms are as they
provide an easy system for defining messages that the other process can then match on and perform operations.

Then there is also a system for managing processes (with other processes). On a basic level there are special messages, we call them signals,
that a process sends to the process that spawned it when it exits. Signals are special because they act on the parent on their own.
So we usually don't define how a dying child should be handled. Though there are process flags that can be set to "trap" an error from child processes.
On the topic of dying together with other processes: a processes can also actively link itself to another process using its `PID`. This is 
done to express the dependency between two ongoing tasks that cannot work without one another. If linked, processes also die together.

Other control structures that can be composed together to form a bigger system are supervisors and the `gen_server`. In erlang we call them 
behaviors. They are patterns provided by OTP that define how a system works but not what it does. That way it is relatively simple to 
use patterns that eliminate boilerplate and provide a sensibel base to build upon.

### supervisors

Supervisors are usually used to build up a supervisor-tree. I think that is really cool because it reflects the actual structure and dependencies 
in the project. This is one of the many places where I feel like erlang solves problem much more concrete and "hands on" than other languages.
There is something about putting actual timeout values when sending messages to other processes or defining hierarchies between components that 
intrigues me and that I don't really see in other languages. But it is something that I will get back on in the section about defensive programming.

Supervisors have the purpose of restarting child processes if they fail. But they allow fine grained control over how it should be done because
that is relevant to the functionality and said hierarchies. Given a tree that looks something like this:
```
   A
 / | \
B  C  D
```
where A is the supervisor. Now we assume C exits abnormally, then depending on how the supervisor is configured multiple things can happen. Here 
are the most common options:

| method         | effect                   |
|----------------|--------------------------|
| `one_for_one`  | C get restarted          |
| `one_for_all`  | B, C and D get restarted |
| `rest_for_one` | C and D get restarted    | 

Processes get started from left to right, that is why only C and D get restarted in the last case. But on the topic of abnormal exits, what happens
to a process that is managed by a supervisor when it exits can also be configured:

| restart policy | normal exit | abnormal exit |
|----------------|-------------|---------------|
| permanent      | restart     | restart       |
| transient      | stay dead   | restart       | 
| temporary      | stay dead   | stay dead     | 

I think this is enough theoretical talk and I for my part do like some code snippets in the blogs I read so here we go.
When using a behavior it is similar to implementing an interface (but it isn't enforced that the "methods" are actually implemented, again, more on 
that later).
`-behavior(BehaviorName)`
For the supervisor the main callbacks that need to be there are `init/1`, `handle_call/3`, `terminate/2`, …
And when spawning workers this is a simplified code snipped:

```erlang
%% "?MODULE" is a shorthand for the current module, so it doesn't need to be repeated all the time
start_link() -> supervisor:start_link({local, ?MODULE}, ?MODULE, []).

init([]) ->
    %% example config for one child
    WorkerSpec = #{
        id => my_worker,
        start => {my_worker, start_link, []},

        %% Always restart
        restart => permanent, 

        %% Give it 5 seconds
        shutdown => 5000,

        %% 2 types of children
        %%  -> workes
        %%  -> supervisor
        type => worker
    },

    %% supervisor config
    SupervisorSpec = #{
        %% restart children that fail
        strategy => one_for_one,

        %% Allow 10fails/minute
        intensity => {10, 60}
    },

    %% start a list of workes in the supervisor
    {ok, {SupervisorSpec, [WorkerSpec]}}.
```
This is just about how much erlang I wrote in the past xD. So far I kept it to minimal examples and writing [gleam](https://gleam.run/) which 
transpiles to erlang and uses the same concepts and building blocks.

### `gen_server`

The `gen_server` behavior models the interaction between clients and servers in the internet within your application. As far as I know it is most 
commonly used to distribute work instead of error handling. It would block the process to do the (possibly expensive) computation and thus block 
the processes from reading new messages in the inbox. Therefore it spawns new processes that do the work and then report back with the result.

## non-defensive programming

This is something that I never really thought about before. In most other programming languages and in rust especially defensive programming is 
the way errors are handled. For me that means that we try to treat errors as normal paths an application can take and not as special cases. So 
the option that an error occurs should always be kept in mind and be handled. With erlang the error solving strategy (errors usually mean that
a process has exited abnormally) is to restart the processes, thus resetting it into a state that is known to be good/valid. If that doesn't 
resolve the error then it will try that again until the supervisor decides that this were too many restarts and that it cannot handle this case.
And as we know the way to deal with that is to exit abnormally and let the parent handle it. This of course is not the entire picture, as 
this wouldn't get us anywhere but to always propagate up but it is an approach I haven't seen elsewhere. And within this concept it all is
coherent. If we look at the atoms as fields of an potentially infinite enum, then a process only "accepts" some of the values as valid commands.
When sent an unexpected message it is then good to tell the supervisor because it indicates that something went wrong elsewhere and that needs
to be addressed. And only the processes higher up in the tree can do so. Given the right patterns it is helpful to panic as error management.

But for type errors and other static checking there still is [Dialyzer](https://www.erlang.org/doc/apps/dialyzer/dialyzer.html).

## sources and further reading

- [learnyousomeerlang](https://learnyousomeerlang.com/)
- [erlang docs](https://www.erldocs.com/18.0/stdlib/gen_server.html#init/1)
- [distributed erlang](https://www.erlang.org/doc/system/distributed.html)

`Sun 8.2.2026 - Lovis Rentsch - EUPLv1.2`
of course no LLM was involved in writing this text
