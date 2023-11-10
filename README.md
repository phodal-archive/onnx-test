# onnx-test

## Setup steps

1.setup

```bash
pip install optimum[exporters,onnxruntime]
```

2.convert onnx

```bash
optimum-cli export onnx --model sentence-transformers/all-MiniLM-L6-v2 all-MiniLM-L6-v2-onnx
```

## Better example

```bash
optimum-cli export onnx -m Helsinki-NLP/opus-mt-zh-en --optimize O2 optus-mt-zh-en-onnx
```

OnnxRuntime

```bash
optimum-cli onnxruntime quantize \
  --avx512 \
  --onnx_model bert-tiny-onnx \
  -o quantized_bert-tiny-onnx
```
