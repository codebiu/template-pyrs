from setuptools import setup, find_packages

setup(
    name='tempalte_bin',
    version='0.1.0',
    author='Your Name',  # 替换为你的信息
    author_email='your.email@example.com',  # 替换为你的信息
    description='A brief description of tempalte_bin',  # 简要描述
    packages=find_packages(where='src'),
    package_dir={'': 'src'},
    install_requires=[
        # 列出项目的依赖项
    ],
)