#!/bin/bash
dropdb --if-exists blog
diesel setup --database-url 'postgres://postgres:admin@localhost/blog'