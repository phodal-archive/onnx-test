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
optimum-cli export onnx -m deepset/roberta-base-squad2 --optimize O2 roberta_base_qa_onnx
```

OnnxRuntime

```
optimum-cli onnxruntime quantize \
  --avx512 \
  --onnx_model roberta_base_qa_onnx \
  -o quantized_roberta_base_qa_onnx
```
