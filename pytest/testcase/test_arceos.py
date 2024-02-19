#!/usr/bin/python3
# -*- coding:utf-8 -*-
import pytest
import re
import allure
import os
import sys
import logging

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
logging.info(BASE_DIR)
sys.path.append(BASE_DIR)

from lib import cmd
from lib import ssh
from lib import excel
from lib import db

from config import *

cmd_list = ["apps/helloworld", "apps/memtest", "apps/exception", "apps/task/yield", "apps/task/parallel", "apps/task/sleep", "apps/task/priority", "apps/task/tls", "apps/net/httpclient", "apps/c/helloworld", "apps/c/memtest", "apps/c/sqlite3", "apps/c/httpclient", "apps/c/pthread/basic", "apps/c/pthread/sleep", "apps/c/pthread/pipe", "apps/c/pthread/parallel"]

@allure.step("测试前置步骤一：SSH登录域控204")
@pytest.fixture(scope='module', name='cmdRun', autouse=True)
def step_setup01():  # 步骤函数命名不能以test_开头，否则将被识别为自动化用例
    logging.info("测试前置步骤一：初始化cmd库")
    logging.info("Setup for class with cmd")
    cmdRun = cmd.cmd()
    yield cmdRun
    logging.info("测试后置步骤：打印日志")


@allure.step("测试步骤一：执行测试")
def step_01(cmdRun, cmdApp):
    _, res = cmdRun.run_cmd('cd /home/runner/work/starry_ext4/starry_ext4 && ./1.sh sdcard && source ~/.bashrc && make A=%s ARCH=riscv64 run' %cmdApp)
    logging.info("res=" + res)
    assert res


@allure.feature("特性（对应敏捷开发中的feature）")
@allure.issue(url="",name="用例对应issuer的链接，若没有可删除此行")
@allure.link(url="",name="用例对应需求的链接，若没有，可删除此行")
@allure.story("故事（对应敏捷开发中的story)")
@allure.severity('用例的级别，一般常用的级别为：blocker（阻塞缺陷），critical（严重缺陷），normal（一般缺陷），minor次要缺陷，trivial（轻微缺陷）')
@allure.title("测试ArceOS基本功能")
@allure.description("测试用例简要描述")
@pytest.mark.parametrize("cmd_list", cmd_list)
@pytest.mark.repeat(1)
def test_arceos(cmdRun, cmd_list):
    """测试内核实时性指标"""
    kpi = step_01(cmdRun, cmd_list)


if __name__ == '__main__':
    pytest.main(['-sv', '--alluredir', 'report/result', 'testcase/test_arceos.py', '--clean-alluredir'])
