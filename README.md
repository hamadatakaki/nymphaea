# Nymphaea

自作VCSの教科書になるような、シンプルなバージョン管理システムを目指しています.
[giraffe](https://gitlab.com/hamadatakaki/giraffe)を書いていて問題に感じたところをフィードバックしました.

## command list

* init
  * 差分記録用のメタデータを初期化する.
* add
  * ワークスペース上の変更のあったファイルを登録する.
* commit *message*
  * 最新のaddされた変更を記録する.

**追加予定のあるコマンド**

* log
  * コミットログを出力する.
* reset *commit_hash*
  * 特定のコミットのワークスペースをロードする.
* checkout *branch*
  * commitを記録するbranchを切り替える.
* branch *name*
  * 新たなbranchを作成する.
* merge *branch1* *branch2*
  * branch1の変更内容をbranch2に移転し、branch1を削除する.（`mv`をcommitで行うイメージ）

## metadata

* .nymphaea/index
* .nymphaea/current_branch
* .nymphaea/commit_metadatas
* .nymphaea/object_hash_table
* .nymphaea/commit_logs/$BRANCH
* .nymphaea/objects/$OBJECT_HASH

### .nymphaea/index
`add`時点で最新の、トップtreeオブジェクトの`object_hash`を記録しておく.
`commit`の時にこのファイルに記録されている情報を元に`commit_logs`の該当ファイルを更新する.

### .nymphaea/current_branch
現在指しているブランチを記録しておく（`git checkout`相当の機能が追加された時用）

### .nymphaea/commit_metadatas
特定のcommitオブジェクトが何本のbranchから参照されているか記録する.
また、`commit_hash`と、そのcommitで生成された`object`のペアを記録する.
前者は`reset`を実行する時に、リセット後のcommitオブジェクトの参照カウントを計算し、それが0になったcommitの関連objectは削除するため.
後者は`reset`コマンドにて、削除するcommitに関して、その`commit_hash`から該当commitで初めて作られたobjectの`object_hash`を知るため.

### .nymphaea/object_hash_table
特定のblobオブジェクトの`object_hash`と、それが含まれる最新の`commit_hash`と、その元ファイルのパスの3つのデータを`object_hash`ごとに記録する.
`add`などで最新のobjectとワークスペースのファイルの差分を取る際、ファイルパスと最新の`commit_hash`から、そのcommitでの指定されたファイルの`object_hash`を知るため.

### .nymphaea/commit_logs/$BRANCH
ブランチごとにcommit履歴を記録しておく.
`log`で表示したいメタデータもついでに記録する.

### .nymphaea/objects/$OBJECT_HASH
各objectを記録しておく.色々と使うだろう.

### メタデータファイルのフォーマット(一部)

```
// .nymphaea/commit_metadatas
[<commit_hash> <ref_count> <object_hash>[ <object_hash>]*\n]*
```

```
// .nymphaea/object_hash_table
[<blob_object_hash> <latest_commit_hash> <file_path>\n]*
```

```
// .nymphaea/commit_logs/$BRANCH
[<commit_hash> <unixtime> <message>\n]*  // 上に行くほど古くなり、末尾に近いほど新しくなる.
```

```
// .nymphaea/objects/$TREE_OBJECT
[(blob|tree) <object_hash> <path>\n]*
```
