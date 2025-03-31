using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 3, 5, 1 }, 2, 4)]
    [DataRow(new[] { 1, 3 }, 2, 0)]
    public void TestMethod1(int[] nums, int k, int exp)
    {
        Assert.AreEqual(exp, sol.PutMarbles(nums, k));
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