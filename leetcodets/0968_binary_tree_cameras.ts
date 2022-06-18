class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

class EnhancedTreeNode {
  parent: EnhancedTreeNode | null;
  val: number;
  childLeft: EnhancedTreeNode | null;
  childRight: EnhancedTreeNode | null;
  coveredByCamera: boolean;

  constructor(
    val?: number,
    left?: EnhancedTreeNode | null,
    right?: EnhancedTreeNode | null,
    parent?: EnhancedTreeNode | null
  ) {
    this.val = val === undefined ? 0 : val;
    this.childLeft = left === undefined ? null : left;
    this.childRight = right === undefined ? null : right;
    this.parent = parent === undefined ? null : parent;
    this.coveredByCamera = false;
  }
}

function addParentPointer(node: TreeNode): EnhancedTreeNode {
  const graphNode = new EnhancedTreeNode(node.val);
  graphNode.childLeft = node.left ? addParentPointer(node.left) : null;
  if (graphNode.childLeft) graphNode.childLeft.parent = graphNode;
  graphNode.childRight = node.right ? addParentPointer(node.right) : null;
  if (graphNode.childRight) graphNode.childRight.parent = graphNode;
  return graphNode;
}

function minCameraCover(root: TreeNode | null): number {
  if (root === null) {
    return 0;
  }
  const enhancedRoot = addParentPointer(root);
  const bfs = () => {
    let values: EnhancedTreeNode[] = [];
    let queue = [enhancedRoot];
    let tmpNode;
    while (queue.length > 0) {
      tmpNode = queue.pop();
      values.push(tmpNode);
      if (tmpNode?.childLeft) {
        queue.unshift(tmpNode?.childLeft);
      }
      if (tmpNode?.childRight) {
        queue.unshift(tmpNode?.childRight);
      }
    }
    return values;
  };

  const queue = bfs().reverse();

  let cameraCount = 0;

  const placeCamera = (node: EnhancedTreeNode) => {
    cameraCount++;
    node.coveredByCamera = true;
    if (node?.childLeft) node.childLeft.coveredByCamera = true;
    if (node?.childRight) node.childRight.coveredByCamera = true;
    if (node?.parent) node.parent.coveredByCamera = true;
  };

  for (const node of queue) {
    if (node.coveredByCamera) {
      continue;
    }
    if (node?.parent) {
      placeCamera(node.parent);
    } else {
      placeCamera(node);
    }
  }

  return cameraCount;
}
