# onnx-test

## Setup steps

1.setup

```bash
pip install optimum[exporters,onnxruntime]
```

2.convert onnx

```bash
optimum-cli export onnx --model Helsinki-NLP/opus-mt-zh-en optus-mt-zh-en-onnx
```

## Better example

```bash
optimum-cli export onnx -m Helsinki-NLP/opus-mt-zh-en --optimize O2 optus-mt-zh-en-onnx
```

OnnxRuntime

```bash
optimum-cli onnxruntime quantize \
  --avx512 \
  --onnx_model optus-mt-zh-en-onnx \
  -o quantized_optus-mt-zh-en-onnx
```
