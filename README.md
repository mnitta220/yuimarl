# Yuimarl Project
Yuimarl Project

## 1. ローカルDB

```bash
cd /c/Projects/yuimarl/01db
docker-compose up -d
docker container exec -it postgresql bash
psql -U yuimarluser -d yuimarl
\i init\01_ddl.sql
\i init\20_nsert_sample_data.sql
\dt
\q
```

## 2. ローカル Spring Boot

```bash
cd /c/Projects/yuimarl_ap
export DB_URL="jdbc:postgresql://localhost:5432/yuimarl"
export DB_USERNAME=yuimarluser
export DB_PASSWORD=yuimarlpass
./mvnw spring-boot:run
./mvnw clean package
java -jar target/yuimarl_ap-1.0.0.jar

cd /c/Projects/yuimarl/02ap
cp ../../yuimarl_ap/target/yuimarl_ap-1.0.0.jar .
docker build -t mnitta220/yuimarl_ap:latest --build-arg JAR_FILE=yuimarl_ap-1.0.0.jar .
docker login -u mnitta220 -p ***
docker image push mnitta220/yuimarl_ap:latest

```

## 3. ローカル docker + Nuxt.js

```bash
cd /c/Projects/yuimarl/01db
docker-compose down
cd /c/Projects/yuimarl/03docker
docker-compose up -d
cd /c/Projects/yuimarl_nuxt
yarn dev
```

## 4. ローカル Kubernetes + Nuxt.js

## 5. AWS

#### Build

```bash
./gradlew clean build
cd build/libs
java -jar ./yuimarl-1.0.0.jar
```

* [http://localhost:8080/hello](http://localhost:8080/hello)


#### Image
##### yuimarl_ap

```bash
cd /c/Projects/yuimarl/infra/ap
cp ../../../yuimarl_ap/target/yuimarl_ap-1.0.0.jar .
docker build -t mnitta220/yuimarl_ap:latest --build-arg JAR_FILE=yuimarl_ap-1.0.0.jar .
docker login -u mnitta220 -p ***
docker image push mnitta220/yuimarl_ap:latest

```

##### yuimarl_nuxt

```bash
cd /c/Projects/yuimarl/infra/nuxt
cp -r ../../../yuimarl_nuxt/dist .
docker build -t mnitta220/yuimarl_nuxt:latest .
docker login -u mnitta220 -p ***
docker image push mnitta220/yuimarl_nuxt:latest

```


#### Kubernetes

```bash
cd /c/Projects/yuimarl/infra/k8s
kubectl apply -f 00_namespace.yaml
kubectl get namespace
kubectl apply -f 01_postgres.yaml
kubectl apply -f 02_api.yaml
kubectl get all -n=yuimarl
kubectl expose deployment yuimarlapi -n=yuimarl --port 8080 --type NodePort
kubectl get service -n=yuimarl
kubectl delete service yuimarlapi -n=yuimarl

kubectl exec -it postgresql-0 -n=yuimarl -- bash
createuser -d -U ympostgresadmin -P -h localhost mywork
createdb -U mywork -h localhost -E UTF8 myworkdb
psql -U mywork -d myworkdb
\i init\10_ddl.sql
\i init\20_nsert_sample_data.sql
\dt
\q

kubectl get configmaps -n=yuimarl
kubectl apply -f 02_local_pv2.yaml -n=yuimarl
kubectl apply -f 03_local_pvc.yaml -n=yuimarl
kubectl apply -f 04_postgres_service.yaml -n=yuimarl
kubectl apply -f 05_postgres_stateful.yaml -n=yuimarl
kubectl get pod -n=yuimarl
kubectl exec -it postgres-ym-0 bash -n=yuimarl


kubectl apply -f replicaset.yaml
kubectl get pods
kubectl apply -f service.yaml
kubectl get svc yuimarl-service
kubectl delete -f service.yaml
kubectl delete -f replicaset.yaml

```
* http://localhost:32511/hello

# ローカル環境構築

* PostgreSQL
* Spring Boot
* nginx
* Vue.js

## Test

```bash
cd /c/Projects/yuimarl/infra/front
docker build -t simple-nginx .
docker run -d --name simple-nginx-container -p 80:80 simple-nginx
```

## Docker for Windows Kubernetes

```bash
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/mandatory.yaml
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/provider/cloud-generic.yaml
```

## Minikube

```bash
minikube start
minikube stop
minikube delete
minikube status
minikube ip
minikube ssh
minikube dashboard

cd /c/Projects/yuimarl/infra/k8s
kubectl apply -f kube_local.yaml
kubectl get pod -n=yuimarl
kubectl exec -it postgres-0 -n=yuimarl -- bash
createuser -d -U ympostgresadmin -P -h localhost mywork
createdb -U mywork -h localhost -E UTF8 myworkdb
psql -U mywork -d myworkdb
\i init\10_ddl.sql
\i init\20_nsert_sample_data.sql
\dt
\q
```

## PostgreSQL

```bash
cd /c/Projects/yuimarl/infra/postgres
$ cat .env
POSTGRES_USER=mywork
POSTGRES_PASSWORD=test01
POSTGRES_DB=myworkdb

docker-compose up -d
docker container exec -it postgresql bash
createuser -d -U postgres -P -h localhost mywork
createdb -U mywork -h localhost -E UTF8 myworkdb
psql -U mywork -d myworkdb
\i init\10_ddl.sql
\i init\20_nsert_sample_data.sql
\dt
\q
```

## Nuxt.js
```bash
yarn create nuxt-app yuimarlv
# install dependencies
$ yarn install

# serve with hot reload at localhost:3000
$ yarn dev

# build for production and launch server
$ yarn build
$ yarn start

# generate static project
$ yarn generate

```

* http://localhost:3000/


## Vue.js
```bash
vue create yuimarlv
```
* Manually select features
* Choose Vue version, Babel, Router, Vuex, Linter / Formatter
* 3.x (Preview)
* Use history mode for router? -> Y
* ESLint with error prevention only
* In dedicated config files
* Save this as a preset for future projects? -> N

```bash
cd yuimarlv
yarn serve
yarn build
```

* http://localhost:8080/

```bash
docker run --name some-nginx -v C:\\Projects\\yuimarlv\\dist:/usr/share/nginx/html:ro -d -p 8080:80 nginx
docker container stop some-nginx
docker container rm some-nginx
```

```bash
cd infra
cp -r ..\dist\ .
docker build -t yuimarlv .
docker run -it --rm -d -p 8080:80 --name web yuimarlv
docker image tag yuimarlv mnitta220/yuimarlv:latest
docker image push mnitta220/yuimarlv:latest
kubectl apply -f 02_nginx_k8s.yaml
kubectl port-forward nginx-pod 8080:80
```

## Spring Boot

```bash
./gradlew bootRun
./gradlew build
java -jar build/libs/yuimarl-1.0.0.jar
java -jar build/libs/yuimarl-1.0.0.jar --debug

cd k8sbook/backend-app
./gradlew clean build
docker build -t mnitta220/backend-app:1.0.0 --build-arg JAR_FILE=build/libs/backend-app-1.0.0.jar .
docker push mnitta220/backend-app:1.0.0

```


# AWS環境構築

## 2-2 EKSクラスター構築

### 2-2-1 ベースリソースの構築

AWSマネージメントコンソール  
管理とガバナンス > CloudFormation > スタックの作成  
スタック  

### 2-2-2 EKSクラスターの構築

```bash
eksctl create cluster \
--vpc-public-subnets subnet-08a77b1ac7a9b5560,subnet-0a6934a0cbefdd443,subnet-0bc73b23d1a717eb4 \
--name eks-work-cluster \
--region ap-northeast-1 \
--version 1.14 \
--nodegroup-name eks-work-nodegroup \
--node-type t2.small \
--nodes 2 \
--nodes-min 2 \
--nodes-max 5
```

eks-work-base  
* RouteTable: rtb-0e5d16551849a5952
* VPC: vpc-00fabdf308d722940
* WorkerSubnets: subnet-08a77b1ac7a9b5560,subnet-0a6934a0cbefdd443,subnet-0bc73b23d1a717eb4

eks-work-rds  
RDSEndpoint	eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com  

```bash
kubectl config get-contexts
kubectl get nodes
cd /c/Projects/yuimarl/infra/eks-env
kubectl apply -f 02_nginx_k8s.yaml
kubectl get pods
kubectl port-forward nginx-pod 8080:80
kubectl delete -f 02_nginx_k8s.yaml
```

## 2-3 データベースのセットアップ

```bash
sudo yum install -y git
sudo amazon-linux-extras install -y postgresql10
cd
pwd
git clone https://github.com/kazusato/k8sbook.git

```

* RDSEndpoint: eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com

シークレットの値  
RdsMasterSecret  
* password: mY1XCInCEb4U}^=G
* dbname: eksworkdb
* engine: postgres
* port: 5432
* dbInstanceIdentifier: eks-work-db
* host: eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com
* username: eksdbadmin

RdsUserSecret  
* password: :tkz(cMcRD4xkxmr
* username: mywork

```bash
createuser -d -U eksdbadmin -P -h eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com mywork
createdb -U mywork -h eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com -E UTF8 myworkdb
psql -U mywork -h eks-work-db.cbgyh8khuxwu.ap-northeast-1.rds.amazonaws.com myworkdb
\i k8sbook/backend-app/scripts/10_ddl.sql
\i k8sbook/backend-app/scripts/20_insert_sample_data.sql
\q

```


## 2-4 APIアプリケーションのビルドとデプロイ

```bash
kubectl apply -f 20_create_namespace_k8s.yaml
kubectl config get-contexts

kubectl config set-context eks-work --cluster eks-work-cluster.ap-northeast-1.eksctl.io --user k8sbook@eks-work-cluster.ap-northeast-1.eksctl.io --namespace eks-work
kubectl config use-context eks-work
kubectl apply -f 21_db_config_k8s.yaml
kubectl apply -f 22_deployment_backend-app_k8s.yaml
kubectl get all
kubectl apply -f 23_service_backend-app_k8s.yaml
kubectl get all

curl -s http://ac97e983e2e9d11eba68606422bf62cc-1769901223.ap-northeast-1.elb.amazonaws.com:8080/health

```

## 2-5 フロントエンドアプリケーションのビルドとデプロイ

```bash
cd /c/Projects/yuimarl_nuxt
BASE_URL=http://ac97e983e2e9d11eba68606422bf62cc-1769901223.ap-northeast-1.elb.amazonaws.com:8080 yarn generate

aws s3 sync dist s3://eks-work-frontend-mnitta220 --delete --include "*" --acl public-read

```

* DistributionID: E1Q71TGS7WBB52
* URL: http://dd3t81onp8pll.cloudfront.net

```bash
aws cloudfront create-invalidation --distribution-id E1Q71TGS7WBB5 --path "/*"
```


## 2-6 バッチアプリケーションのビルドとデプロイ

## 2-7 環境の破棄
### 2-7-2 リソース破棄

```bash
kubectl delete service backend-app-service
kubectl delete deployment backend-app
kubectl delete cronjob batch-app
kubectl get all
eksctl delete cluster --name eks-work-cluster
aws s3 rm s3://eks-work-frontend-mnitta220 --recursive

```


### 2-7-3 EKSクラスター削除

```bash
kubectl delete service backend-app-service
kubectl delete deployment backend-app
kubectl delete cronjob batch-app
eksctl delete cluster --name eks-work-cluster

```

* Qiita マークダウン記法 一覧表・チートシート  
  [https://qiita.com/kamorits/items/6f342da395ad57468ae3](https://qiita.com/kamorits/items/6f342da395ad57468ae3)

* 基礎から応用までじっくり学ぶECS Fargateを利用したコンテナ環境構築
  https://www.youtube.com/watch?v=wnhgcJRsRd0
  https://dev.classmethod.jp/articles/cloudformation-fargate/
