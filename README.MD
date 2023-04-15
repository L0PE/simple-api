# Simple Message API

## Responsibility

Store and returns simple messages

## Haw to deploy

1. Build lambda images
    ```shell
      cd CreateMessage
      cargo lambda build --release --arm64
      cd ../GetMessages
      cargo lambda build --release --arm64
      cd ..
    ```
2. Deploy existing application
    ```shell
      sam deploy
    ```

3. You can go to the AWS management console and get Api Gateway url
