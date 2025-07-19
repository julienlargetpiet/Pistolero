# Pistolero

![logo](logo.jpg)

# Description

A simple terminal programm to track your tasks.

# Use it

Clone it in `your_path`

```
$ cd your_path && rustc main.rs -o pistolero
```

You can now create a bash func like in your `.bashrc`:

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

## Example

```
$ pistolero see_sort_date "order:asc,n:8,lock:01-01-2012|12-12-2040"
     TASK NAME     |     STATUS      |     LEVEL      |     DEADLINE
Task5              | false           | 4              | 12-12-2012
Task2              | false           | 2              | 02-01-2026
Task3              | true            | 2              | 01-11-2026
Task1              | false           | 2              | 02-12-2026
Task4              | false           | 1              | 01-11-2035
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





