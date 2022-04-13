#!/usr/bin/env python3

import os
from minio import Minio

BUCKET_NAME = 'instaget'

def upload(storage_client: Minio, object_name: str, bin_path: str):
    file_stat = os.stat(bin_path)
    with open(bin_path, 'rb') as buff:
        storage_client.remove_object(bucket_name=BUCKET_NAME, object_name=object_name)
        result = storage_client.put_object(bucket_name=BUCKET_NAME, object_name=object_name, data=buff, length=file_stat.st_size)
        print('upload result ', result.object_name)

if __name__ == '__main__':
    storage_endpoint = os.getenv('STORAGE_ENDPOINT')
    storage_access_key = os.getenv('STORAGE_ACCESS_KEY')
    storage_secret_key = os.getenv('STORAGE_SECRET_KEY')
    upload_for = os.getenv('UPLOAD_FOR')

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
        upload(client, object_name=object_name, bin_path=osx_bin_path)
        print('uploading osx bin succeed.....')
    else:
        print('uploading linux bin')
        object_name = 'linux/instaget'
        linux_bin_path = os.path.join(cur_dir, 'build-result/linux/x86_64-unknown-linux-gnu/debug/instaget')
        upload(client, object_name=object_name, bin_path=linux_bin_path)
        print('uploading linux bin succeed.....')