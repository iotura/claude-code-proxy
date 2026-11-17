#!/usr/bin/env python3
"""Write Claude Code settings for the Kaggle models advertised by this proxy."""

import json
import re
import sys
import urllib.request


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} BASE_URL OUTPUT_FILE", file=sys.stderr)
        return 2

    base_url, output_file = sys.argv[1:]
    request = urllib.request.Request(
        f"{base_url.rstrip('/')}/v1/models",
        headers={"x-api-key": "local-kaggle-proxy"},
    )
    with urllib.request.urlopen(request, timeout=5) as response:
        payload = json.load(response)

    options = []
    for item in payload.get("data", []):
        model_id = item.get("id", "")
        if not model_id.startswith("kaggle/"):
            continue
        upstream_id = model_id.removeprefix("kaggle/")
        pieces = upstream_id.split("/")
        name = " ".join(
            part.replace("@default", " default").replace("-", " ").title()
            for part in pieces
        )
        name = re.sub(r"\s+", " ", name).strip()
        options.append(
            {
                "model": model_id,
                "label": name,
                "description": f"Kaggle Model Proxy · {upstream_id}",
            }
        )

    if not options:
        raise RuntimeError("the proxy did not advertise any Kaggle models")

    settings = {
        "modelPicker": {
            "options": options,
            "replaceBuiltInOptions": True,
        }
    }
    with open(output_file, "w", encoding="utf-8") as output:
        json.dump(settings, output, ensure_ascii=False)
        output.write("\n")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:
        print(f"could not build Kaggle model picker: {error}", file=sys.stderr)
        raise SystemExit(1)
