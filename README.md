# onnx-test

## Setup steps

1.setup

```bash
pip install optimum[exporters]
```

2.convert onnx

```bash
optimum-cli export onnx --model  Helsinki-NLP/opus-mt-zh-en optus-mt-zh-en-onnx
```

