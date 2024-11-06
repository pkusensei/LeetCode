using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    // readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        CBTInserter cBTInserter = new CBTInserter(TreeNode.Make([1, 2]));
        Assert.AreEqual(1, cBTInserter.Insert(3));  // return 1
        Assert.AreEqual(2, cBTInserter.Insert(4));  // return 2
        Assert.AreEqual("[1,2,3,4]", cBTInserter.Get_root().ToString()); // return [1, 2, 3, 4]
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