# Pistolero

![logo](logo.jpg)

# Description

A simple terminal programm to track your tasks.

# Use it

Clone it in `your_path`

```
$ cd your_path && rustc main.rs -o pistolero
```

You can now create a bash func like:

```
pistolero() {
  (cd your_path && ./out "$@")
}
```

# Commands

## ADD TASK 

```
pistolero taskname 'name:actual_name,status:false_or_true,level:u8,deadline:day-month-year'
```

## REMOVE TASK

```
pistolero remove actual_name
```

## UPDATE STATUS

```
pistolero updatestatus false_or_true
```

## UPDATE LEVEL 

```
pistolero updatelevel u8
```

## UPDATE DEADLINE

```
pistolero updatedeadline 'day-month-year'
```

## SEE ALL TASKS 

```
pistolero see
```

## SEE DATE SORTED METHODS 

```
pistolero see_sort_date 'lock:min_date|max_date,order:asc_or_desc,n:number_of_tasks_to_print'
```

## SEE LEVEL SORTED METHODS

```
pistolero see_sort_date 'lock:min_date|max_date,order:asc_or_desc,n:number_of_tasks_to_print'
```

## HELP

```
pistolero help
```

## NOTE 

- you can vary the order of parameters, like name,deadline,status,level is also fine
- date system is Gregorian"





