#!/usr/bin/python3
# -*- coding:utf-8 -*-
import pymysql
import datetime


uniCmdList = [
    "apps/helloworld", 
    "apps/memtest", 
    "apps/exception", 
    "apps/task/yield", 
    "apps/task/parallel", 
    "apps/task/sleep", 
    "apps/task/priority", 
    "apps/task/tls", 
    "apps/net/httpclient", 
    "apps/c/helloworld", 
    "apps/c/memtest", 
    "apps/c/sqlite3", 
    "apps/c/httpclient", 
    "apps/c/pthread/basic", 
    "apps/c/pthread/sleep", 
    "apps/c/pthread/pipe", 
    "apps/c/pthread/parallel"
    ]

monoCmdList = [
    "apps/oscomp"
    ]