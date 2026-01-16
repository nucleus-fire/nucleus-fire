<n:model list="crate::logic::tag::Tag::query().all().await.unwrap_or_default()">
<div class="container">
    <h1>All Tags</h1>
    <ul>
        <n:for item="tag" in="list">
            <li>
                <a href="/tags/{ tag.id }">View { tag.id }</a>
            </li>
        </n:for>
    </ul>
    <a href="/tags/new">New Tag</a>
</div>
</n:model>