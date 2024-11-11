helm install kibana elastic/kibana -n elastic \
  --set elasticsearchHosts=https://elasticsearch-master:9200 \
  --set service.type=ClusterIP \
  --set elasticsearch.ssl.verificationMode=none \
  --set xpack.security.enabled=false
