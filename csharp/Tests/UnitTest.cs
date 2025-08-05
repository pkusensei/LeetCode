using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, new[] { 4, 2, 5 }, new[] { 3, 5, 4 })]
    public void TestMethod1(int exp, int[] f, int[] b)
    {
        Assert.AreEqual(exp, sol.NumOfUnplacedFruits(f, b));
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