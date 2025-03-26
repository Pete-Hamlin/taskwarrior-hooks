# taskwarrior hooks

A selection of taskwarrior hooks to help with task management:

- [Count inbox/priority tasks](./src/bin/hook_filter_count.rs)
- [Enforce a maximum for priority tasks](./src/bin/hook_filter_priority.rs)
- [Scheduled-recur](./src/bin/hook_scheduled_recur.rs)

## Install

Run the following to build the binaries and install the hook scripts:

```sh
just install
just hooks
```

## scheduled-recur

Reschedule tasks, instead of completing them. Task will be rescheduled today + (given duration).

This comes from [mrVanDalo/taskwarrior-hooks](https://github.com/mrVanDalo/taskwarrior-hooks) originally, full credit to the original author.

### Setup

Add this UDA to your `~/.taskrc`:

```sh
# scheduled_recur
uda.scheduled_recur.type=duration
uda.scheduled_recur.label=Scheduled Recurance
# END scheduled_recur
```

and [this](./hooks/on-update.reschedule.sh) hook to `~/.task/hooks/on-modify.reschedule.sh`

```sh
#!/usr/bin/bash
hook_scheduled_recur
```

### Usage

Duration are set using [ISO8601](https://en.wikipedia.org/wiki/ISO_8601#Durations). For example : `scheduled_recur:P1D` or `scheduled_recur:P1DT8H`
