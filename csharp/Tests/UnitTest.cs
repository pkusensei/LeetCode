using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("aabaaaacaabc", 2, 8)]
    [DataRow("a", 1, -1)]
    public void TestMethod1(string s, int k, int exp)
    {
        Assert.AreEqual(exp, sol.TakeCharacters(s, k));
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