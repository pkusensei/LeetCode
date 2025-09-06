using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(23, new[] { 1, 3, 2, 2, 2, 3, 4, 3, 1 })]
    public void TestMethod1(int exp, int[] b)
    {
        Assert.AreEqual(exp, sol.RemoveBoxes(b));
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