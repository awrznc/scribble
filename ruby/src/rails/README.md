Ruby on Rails
===

## Default URL

http://localhost:3000/

## Command

※ docker-compose run で起動する場合はポートを指定すること

```bash
# init
rails new app_name

# run
rails s -p 3000 -b '0.0.0.0'

# generate controller
rails generate controller {{controller_name}} {{action_name}}

# generate model
rails generate model {{model_name (postsの場合はPostを指定する)}} {{column_name}}:{{type}}

# generate table
rails db:migrate

# console
rails console
> post = {{ClassName}}.new({{column_name}}: {{value}})
> post.save
> post.{{column_name}}

> # read first data
> Post.first

> # read all data
> Post.all
```
