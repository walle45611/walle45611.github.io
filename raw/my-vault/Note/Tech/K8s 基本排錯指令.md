```
kubectl rollout restart deployment argocd-image-updater -n argocd
kubectl logs argocd-image-updater-6f6696b6c6-7224j -n argocd
```