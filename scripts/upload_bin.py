#!/usr/bin/env python3

import os
from minio import Minio

storage_endpoint = os.getenv('STORAGE_ENDPOINT')
storage_access_key = os.getenv('STORAGE_ACCESS_KEY')
storage_secret_key = os.getenv('STORAGE_SECRET_KEY')
upload_for = os.getenv('UPLOAD_FOR')

bucket_name = 'instaget'

client = Minio(
    endpoint=storage_endpoint,
    access_key=storage_access_key,
    secret_key=storage_secret_key,
    secure=False,
)

cur_dir = os.getcwd()

if upload_for == 'OSX':
    print('uploading osx bin')
    object_name = 'osx/instaget'
    osx_bin_path = os.path.join(cur_dir, 'build-result/osx/x86_64-apple-darwin/debug/instaget')
    file_stat = os.stat(osx_bin_path)
    with open(osx_bin_path, 'rb') as osx_buff:
        client.remove_object(bucket_name=bucket_name, object_name=object_name)
        result = client.put_object(bucket_name=bucket_name, object_name=object_name, data=osx_buff, length=file_stat.st_size)
        print('upload result ', result.object_name)
else:
    print('uploading linux bin')
    object_name = 'linux/instaget'
    linux_bin_path = os.path.join(cur_dir, 'build-result/linux/x86_64-unknown-linux-gnu/debug/instaget')
    file_stat = os.stat(linux_bin_path)
    with open(linux_bin_path, 'rb') as linux_buff:
        client.remove_object(bucket_name=bucket_name, object_name=object_name)
        result = client.put_object(bucket_name=bucket_name, object_name=object_name, data=linux_buff, length=file_stat.st_size)
        print('upload result ', result.object_name)
