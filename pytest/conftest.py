#!/usr/bin/python3
# -*- coding:utf-8 -*-
import pytest
import allure
import os
import sys


@pytest.fixture(scope="session", autouse=True)
def prepare_before_all_case():
    print("初始化步骤：在所有的自动化脚本之前自动执行一次")
    pass