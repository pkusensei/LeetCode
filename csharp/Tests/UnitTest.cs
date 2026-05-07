using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("dba", new[] { 0, 1, 2, 3, 4, 3, 4 })]
    public void TestMethod1(string exp, int[] v)
    {
        TreeNode t = TreeNode.MakeInt(v);
        Assert.AreEqual(exp, sol.SmallestFromLeaf(t));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}